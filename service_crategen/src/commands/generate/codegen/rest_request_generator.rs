use ::Service;
use botocore::{Member, Operation, Shape, ShapeType};
use inflector::Inflector;
use regex::{Captures, Regex};
use super::generate_field_name;

// Add request headers for any shape members marked as headers
pub fn generate_headers(service: &Service, operation: &Operation) -> Option<String> {
    if operation.input.is_none() {
        return None;
    }

    let shape = service.get_shape(&operation.input.as_ref().unwrap().shape).unwrap();

    Some(shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            if member.location.is_none() {
                return None;
            }
            match &member.location.as_ref().unwrap()[..] {
                "header" => {
                    if shape.required(member_name) {
                        Some(format!("request.add_header(\"{location_name}\", 
                                      &input.{field_name});",
                                     location_name = member.location_name.as_ref().unwrap(),
                                     field_name = generate_field_name(member_name)))
                    } else {
                        Some(format!("
                        if let Some(ref {field_name}) = 
                                      input.{field_name} {{
                                      request.add_header(\"{location_name}\", 
                                      &{field_name}.to_string());
                        }}",
                                     location_name = member.location_name.as_ref().unwrap(),
                                     field_name = generate_field_name(member_name)))
                    }
                }
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join("\n"))
}

pub fn generate_params_loading_string(service: &Service, operation: &Operation) -> Option<String> {
    if operation.input.is_none() {
        return None;
    }

    let input_type = operation.input_shape();
    let input_shape = service.get_shape(input_type).unwrap();

    // Construct a list of strings which will be used to load request
    // parameters from the input struct into a `Params` vec, which will
    // then be added to the request.
    let mut param_strings = generate_shape_member_param_strings(service, input_shape);
    param_strings.append(&mut generate_static_param_strings(operation));

    let load_params = match param_strings.len() {
        0 => "".to_owned(),
        _ => {
            format!("let mut params = Params::new();
                {param_strings}
                request.set_params(params);",
                    param_strings = param_strings.join("\n"))
        }
    };

    Some(load_params)
}

pub fn generate_uri_formatter(request_uri: &str,
                              service: &Service,
                              operation: &Operation)
                              -> Option<String> {
    if operation.input.is_some() {
        let input_shape = &service.get_shape(operation.input_shape()).unwrap();
        let uri_strings = generate_shape_member_uri_strings(input_shape);

        if !uri_strings.is_empty() {
            return Some(format!("let request_uri = format!(\"{request_uri}\", {uri_strings});",
                                request_uri = generate_snake_case_uri(request_uri),
                                uri_strings = uri_strings.join(", ")));

        }
    }

    Some(format!("let request_uri = \"{request_uri}\";",
                 request_uri = request_uri))
}

fn generate_shape_member_uri_strings(shape: &Shape) -> Vec<String> {
    shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            generate_member_format_string(&generate_field_name(member_name), member)
        })
        .collect::<Vec<String>>()
}

fn generate_member_format_string(member_name: &str, member: &Member) -> Option<String> {
    match member.location {
        Some(ref x) if x == "uri" => {
            match member.location_name {
                Some(ref loc_name) => {
                    Some(format!(
                        "{member_name} = input.{field_name}",
                        field_name = member_name,
                        member_name = loc_name.to_snake_case(),
                    ))
                }
                None => {
                    Some(format!(
                        "{member_name} = input.{field_name}",
                        field_name = member_name,
                        member_name = member_name.to_snake_case(),
                    ))
                }
            }
        }
        Some(_) | None => None,
    }
}

fn generate_static_param_strings(operation: &Operation) -> Vec<String> {

    // botocore includes + for greedy parameters and we don't care about it
    let (_, maybe_params) = parse_query_string(&operation.http.request_uri);

    let mut static_params = vec![];

    // Sometimes the key has the query param already set, like "list-type=2"
    // These need to get parsed out of the URI and added to the params map
    // with the other "queryparam" members
    if let Some(ref key) = maybe_params {
        match key.find('=') {
            Some(_) => {
                let key_val_vec: Vec<&str> = key.split('=').collect();
                static_params.push(format!("params.put(\"{}\", \"{}\");",
                                           key_val_vec[0],
                                           key_val_vec[1]))
            }
            None => static_params.push(format!("params.put_key(\"{}\");", key)),
        }
    };

    static_params
}

fn generate_snake_case_uri(request_uri: &str) -> String {
    lazy_static! {
        static ref URI_ARGS_REGEX: Regex = Regex::new(r"\{([\w\d]+)\}").unwrap();
    }

    URI_ARGS_REGEX.replace_all(request_uri, |caps: &Captures| {
            format!("{{{}}}",
                    caps.get(1).map(|c| Inflector::to_snake_case(c.as_str())).unwrap())
        })
        .to_string()
}

fn generate_shape_member_param_strings(service: &Service, shape: &Shape) -> Vec<String> {
    shape.members
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|(member_name, member)| {
            member.location.as_ref().and_then(|loc| {
                if !member.deprecated() && loc == "querystring" {
                    let member_shape = service.shape_for_member(member).unwrap();
                    Some(generate_param_load_string(member_name,
                                                    member_shape,
                                                    member,
                                                    shape.required(member_name)))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<String>>()
}

fn generate_param_load_string(member_name: &str,
                              member_shape: &Shape,
                              member: &Member,
                              is_required: bool)
                              -> String {
    let field_name = generate_field_name(member_name);
    let param_name = match member.location_name {
        Some(ref location) => location.to_owned(),
        None => member_name.to_owned(),
    };
    match (member_shape.shape_type, is_required) {
        (ShapeType::List, true) => {
            format!("for item in input.{field_name}.iter() {{
                     params.put(\"{param_name}\", item);
                }}",
                    param_name = param_name,
                    field_name = field_name)
        }
        (ShapeType::List, false) => {
            format!(
                "if let Some(ref x) = input.{field_name} {{
                    for item in x.iter() {{
                        params.put(\"{param_name}\", item);
                    }}
                }}",
                param_name = param_name,
                field_name = field_name,
            )
        }
        (ShapeType::Map, true) => {
            format!("for (key, val) in input.{field_name}.iter() {{
                     params.put(key, val);
                }}",
                    field_name = field_name)
        }
        (ShapeType::Map, false) => {
            format!(
                "if let Some(ref x) = input.{field_name} {{
                    for (key, val) in x.iter() {{
                        params.put(key, val);
                    }}
                }}",
                field_name = field_name,
            )
        }
        (_, true) => {
            format!("params.put(\"{param_name}\", &input.{field_name});",
                    param_name = param_name,
                    field_name = field_name)
        }
        (_, false) => {
            format!(
                "if let Some(ref x) = input.{field_name} {{
                    params.put(\"{param_name}\", x);
                }}",
                param_name = param_name,
                field_name = field_name,
            )
        }
    }
}


pub fn parse_query_string(uri: &str) -> (String, Option<String>) {
    // botocore query strings for S3 are variations on "/{Bucket}/{Key+}?foobar"
    // the query string needs to be split out and put in the params hash,
    // and the + isn't useful information for us
    let base_uri = uri.replace("+", "");
    let parts: Vec<&str> = base_uri.split('?').collect();

    match parts.len() {
        1 => (parts[0].to_owned(), None),
        2 => (parts[0].to_owned(), Some(parts[1].to_owned())),
        _ => panic!("Unknown uri structure {}", uri),
    }
}
