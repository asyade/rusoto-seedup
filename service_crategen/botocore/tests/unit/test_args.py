#!/usr/bin/env
# Copyright 2016 Amazon.com, Inc. or its affiliates. All Rights Reserved.
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
import botocore.config
from tests import unittest
import mock

from botocore import args
from botocore.config import Config


class TestCreateClientArgs(unittest.TestCase):
    def setUp(self):
        self.args_create = args.ClientArgsCreator(None, None, None, None, None)

    def test_compute_s3_configuration(self):
        scoped_config = {}
        client_config = None
        self.assertIsNone(
            self.args_create.compute_s3_config(
                scoped_config, client_config))

    def test_compute_s3_config_only_scoped_config(self):
        scoped_config = {
            's3': {'use_accelerate_endpoint': True},
        }
        client_config = None
        self.assertEqual(
            self.args_create.compute_s3_config(scoped_config, client_config),
            {'use_accelerate_endpoint': True}
        )

    def test_client_s3_accelerate_from_varying_forms_of_true(self):
        scoped_config= {'s3': {'use_accelerate_endpoint': 'True'}}
        client_config = None

        self.assertEqual(
            self.args_create.compute_s3_config(
                {'s3': {'use_accelerate_endpoint': 'True'}},
                client_config=None),
            {'use_accelerate_endpoint': True}
        )
        self.assertEqual(
            self.args_create.compute_s3_config(
                {'s3': {'use_accelerate_endpoint': 'true'}},
                client_config=None),
            {'use_accelerate_endpoint': True}
        )
        self.assertEqual(
            self.args_create.compute_s3_config(
                {'s3': {'use_accelerate_endpoint': True}},
                client_config=None),
            {'use_accelerate_endpoint': True}
        )

    def test_client_s3_accelerate_from_client_config(self):
        self.assertEqual(
            self.args_create.compute_s3_config(
                scoped_config=None,
                client_config=Config(s3={'use_accelerate_endpoint': True})
            ),
            {'use_accelerate_endpoint': True}
        )

    def test_client_s3_accelerate_client_config_overrides_scoped(self):
        self.assertEqual(
            self.args_create.compute_s3_config(
                scoped_config={'s3': {'use_accelerate_endpoint': False}},
                client_config=Config(s3={'use_accelerate_endpoint': True})
            ),
            # client_config beats scoped_config
            {'use_accelerate_endpoint': True}
        )

    def test_client_s3_dualstack_handles_varying_forms_of_true(self):
        scoped_config= {'s3': {'use_dualstack_endpoint': 'True'}}
        client_config = None

        self.assertEqual(
            self.args_create.compute_s3_config(
                {'s3': {'use_dualstack_endpoint': 'True'}},
                client_config=None),
            {'use_dualstack_endpoint': True}
        )
        self.assertEqual(
            self.args_create.compute_s3_config(
                {'s3': {'use_dualstack_endpoint': 'true'}},
                client_config=None),
            {'use_dualstack_endpoint': True}
        )
        self.assertEqual(
            self.args_create.compute_s3_config(
                {'s3': {'use_dualstack_endpoint': True}},
                client_config=None),
            {'use_dualstack_endpoint': True}
        )

    def test_max_pool_from_client_config_forwarded_to_endpoint_creator(self):
        args_create = args.ClientArgsCreator(
            mock.Mock(), None, None, None, None)
        config = botocore.config.Config(max_pool_connections=20)
        service_model = mock.Mock()
        service_model.metadata = {
            'serviceFullName': 'MyService',
            'protocol': 'query'
        }
        service_model.operation_names = []
        bridge = mock.Mock()
        bridge.resolve.return_value = {
            'region_name': 'us-west-2', 'signature_version': 'v4',
            'endpoint_url': 'https://ec2/',
            'signing_name': 'ec2', 'signing_region': 'us-west-2',
            'metadata': {}}
        with mock.patch('botocore.args.EndpointCreator') as m:
            args_create.get_client_args(
                service_model, 'us-west-2', True, 'https://ec2/', True,
                None, {}, config, bridge)
            m.return_value.create_endpoint.assert_called_with(
                mock.ANY, endpoint_url='https://ec2/', region_name='us-west-2',
                response_parser_factory=None, timeout=(60, 60), verify=True,
                max_pool_connections=20, proxies=None
            )

    def test_proxies_from_client_config_forwarded_to_endpoint_creator(self):
        args_create = args.ClientArgsCreator(
            mock.Mock(), None, None, None, None)
        proxies = {'http': 'http://foo.bar:1234',
                   'https': 'https://foo.bar:4321'}
        config = botocore.config.Config(proxies=proxies)
        service_model = mock.Mock()
        service_model.metadata = {
            'serviceFullName': 'MyService',
            'protocol': 'query'
        }
        service_model.operation_names = []
        bridge = mock.Mock()
        bridge.resolve.return_value = {
            'region_name': 'us-west-2', 'signature_version': 'v4',
            'endpoint_url': 'https://ec2/',
            'signing_name': 'ec2', 'signing_region': 'us-west-2',
            'metadata': {}}
        with mock.patch('botocore.args.EndpointCreator') as m:
            args_create.get_client_args(
                service_model, 'us-west-2', True, 'https://ec2/', True,
                None, {}, config, bridge)
            m.return_value.create_endpoint.assert_called_with(
                mock.ANY, endpoint_url='https://ec2/', region_name='us-west-2',
                response_parser_factory=None, timeout=(60, 60), verify=True,
                proxies=proxies, max_pool_connections=10
            )

    def test_s3_with_endpoint_url_still_resolves_region(self):
        self.args_create = args.ClientArgsCreator(
            mock.Mock(), None, None, None, None)
        service_model = mock.Mock()
        service_model.endpoint_prefix = 's3'
        service_model.metadata = {'protocol': 'rest-xml'}
        config = botocore.config.Config()
        bridge = mock.Mock()
        bridge.resolve.side_effect = [
            {
                'region_name': None, 'signature_version': 's3v4',
                'endpoint_url': 'http://other.com/', 'signing_name': 's3',
                'signing_region': None, 'metadata': {}
            },
            {
                'region_name': 'us-west-2', 'signature_version': 's3v4',
                'enpoint_url': 'https://s3-us-west-2.amazonaws.com',
                'signing_name': 's3', 'signing_region': 'us-west-2',
                'metadata': {}
            }
        ]
        client_args = self.args_create.get_client_args(
            service_model, 'us-west-2', True, 'http://other.com/', True, None,
            {}, config, bridge)
        self.assertEqual(
            client_args['client_config'].region_name, 'us-west-2')

    def test_region_does_not_resolve_if_not_s3_and_endpoint_url_provided(self):
        self.args_create = args.ClientArgsCreator(
            mock.Mock(), None, None, None, None)
        service_model = mock.Mock()
        service_model.endpoint_prefix = 'ec2'
        service_model.metadata = {'protocol': 'query'}
        config = botocore.config.Config()
        bridge = mock.Mock()
        bridge.resolve.side_effect = [{
            'region_name': None, 'signature_version': 'v4',
            'endpoint_url': 'http://other.com/', 'signing_name': 'ec2',
            'signing_region': None, 'metadata': {}
        }]
        client_args = self.args_create.get_client_args(
            service_model, 'us-west-2', True, 'http://other.com/', True, None,
            {}, config, bridge)
        self.assertEqual(client_args['client_config'].region_name, None)

    def test_provide_retry_config(self):
        self.args_create = args.ClientArgsCreator(
            mock.Mock(), None, None, None, None)
        service_model = mock.Mock()
        service_model.endpoint_prefix = 'ec2'
        service_model.metadata = {'protocol': 'query'}
        config = botocore.config.Config(
            retries={'max_attempts': 10}
        )
        bridge = mock.Mock()
        bridge.resolve.side_effect = [{
            'region_name': None, 'signature_version': 'v4',
            'endpoint_url': 'http://other.com/', 'signing_name': 'ec2',
            'signing_region': None, 'metadata': {}
        }]
        client_args = self.args_create.get_client_args(
            service_model, 'us-west-2', True, 'https://ec2/', True, None,
            {}, config, bridge)
        self.assertEqual(
            client_args['client_config'].retries, {'max_attempts': 10})
