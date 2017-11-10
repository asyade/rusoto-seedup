# Copyright 2015 Amazon.com, Inc. or its affiliates. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"). You
# may not use this file except in compliance with the License. A copy of
# the License is located at
#
# http://aws.amazon.com/apache2.0/
#
# or in the "license" file accompanying this file. This file is
# distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF
# ANY KIND, either express or implied. See the License for the specific
# language governing permissions and limitations under the License.
import jmespath

import botocore.session
from botocore.utils import ArgumentGenerator


def test_lint_waiter_configs():
    session = botocore.session.get_session()
    for service_name in session.get_available_services():
        client = session.create_client(service_name, 'us-east-1')
        service_model = client.meta.service_model
        for waiter_name in client.waiter_names:
            yield _lint_single_waiter, client, waiter_name, service_model


def _lint_single_waiter(client, waiter_name, service_model):
    try:
        waiter = client.get_waiter(waiter_name)
        # The 'acceptors' property is dynamic and will create
        # the acceptor configs when first accessed.  This is still
        # considered a failure to construct the waiter which is
        # why it's in this try/except block.
        # This catches things like:
        # * jmespath expression compiles
        # * matcher has a known value
        acceptors = waiter.config.acceptors
    except Exception as e:
        raise AssertionError("Could not create waiter '%s': %s"
                             % (waiter_name, e))
    operation_name = waiter.config.operation
    # Needs to reference an existing operation name.
    if operation_name not in service_model.operation_names:
        raise AssertionError("Waiter config references unknown "
                             "operation: %s" % operation_name)
    # Needs to have at least one acceptor.
    if not waiter.config.acceptors:
        raise AssertionError("Waiter config must have at least "
                             "one acceptor state: %s" % waiter.name)
    op_model = service_model.operation_model(operation_name)
    for acceptor in acceptors:
        _validate_acceptor(acceptor, op_model, waiter.name)


def _validate_acceptor(acceptor, op_model, waiter_name):
    if acceptor.matcher.startswith('path'):
        expression = acceptor.argument
        # The JMESPath expression should have the potential to match something
        # in the response shape.
        output_shape = op_model.output_shape
        assert output_shape is not None, (
            "Waiter '%s' has JMESPath expression with no output shape: %s"
            % (waiter_name, op_model))
        # We want to check if the JMESPath expression makes sense.
        # To do this, we'll generate sample output and evaluate the
        # JMESPath expression against the output.  We'll then
        # check a few things about this returned search result.
        search_result = _search_jmespath_expression(expression, op_model)
        if not search_result:
            raise AssertionError("JMESPath expression did not match "
                                 "anything for waiter '%s': %s"
                                 % (waiter_name, expression))
        if acceptor.matcher in ['pathAll', 'pathAny']:
            assert isinstance(search_result, list), \
                    ("Attempted to use '%s' matcher in waiter '%s' "
                     "with non list result in JMESPath expression: %s"
                     % (acceptor.matcher, waiter_name, expression))


def _search_jmespath_expression(expression, op_model):
    arg_gen = ArgumentGenerator(use_member_names=True)
    sample_output = arg_gen.generate_skeleton(op_model.output_shape)
    search_result = jmespath.search(expression, sample_output)
    return search_result
