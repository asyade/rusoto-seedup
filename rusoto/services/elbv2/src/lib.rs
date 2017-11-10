
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

//! <fullname>Elastic Load Balancing</fullname> <p>A load balancer distributes incoming traffic across targets, such as your EC2 instances. This enables you to increase the availability of your application. The load balancer also monitors the health of its registered targets and ensures that it routes traffic only to healthy targets. You configure your load balancer to accept incoming traffic by specifying one or more listeners, which are configured with a protocol and port number for connections from clients to the load balancer. You configure a target group with a protocol and port number for connections from the load balancer to the targets, and with health check settings to be used when checking the health status of the targets.</p> <p>Elastic Load Balancing supports two types of load balancers: Classic Load Balancers and Application Load Balancers. A Classic Load Balancer makes routing and load balancing decisions either at the transport layer (TCP/SSL) or the application layer (HTTP/HTTPS), and supports either EC2-Classic or a VPC. An Application Load Balancer makes routing and load balancing decisions at the application layer (HTTP/HTTPS), supports path-based routing, and can route requests to one or more ports on each EC2 instance or container instance in your virtual private cloud (VPC). For more information, see the <a href="http://docs.aws.amazon.com/elasticloadbalancing/latest/userguide/">Elastic Load Balancing User Guide</a>.</p> <p>This reference covers the 2015-12-01 API, which supports Application Load Balancers. The 2012-06-01 API supports Classic Load Balancers.</p> <p>To get started, complete the following tasks:</p> <ol> <li> <p>Create an Application Load Balancer using <a>CreateLoadBalancer</a>.</p> </li> <li> <p>Create a target group using <a>CreateTargetGroup</a>.</p> </li> <li> <p>Register targets for the target group using <a>RegisterTargets</a>.</p> </li> <li> <p>Create one or more listeners for your load balancer using <a>CreateListener</a>.</p> </li> <li> <p>(Optional) Create one or more rules for content routing based on URL using <a>CreateRule</a>.</p> </li> </ol> <p>To delete an Application Load Balancer and its related resources, complete the following tasks:</p> <ol> <li> <p>Delete the load balancer using <a>DeleteLoadBalancer</a>.</p> </li> <li> <p>Delete the target group using <a>DeleteTargetGroup</a>.</p> </li> </ol> <p>All Elastic Load Balancing operations are idempotent, which means that they complete at most one time. If you repeat an operation, it succeeds.</p>
//!
//! If you're using the service, you're probably looking for [ElbClient](struct.ElbClient.html) and [Elb](trait.Elb.html).

extern crate hyper;
extern crate rusoto_core;
extern crate xml;

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            
