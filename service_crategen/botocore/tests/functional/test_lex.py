# Copyright 2017 Amazon.com, Inc. or its affiliates. All Rights Reserved.
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
import mock
from datetime import datetime

from tests import BaseSessionTest


class TestLex(BaseSessionTest):
    def setUp(self):
        super(TestLex, self).setUp()
        self.region = 'us-west-2'
        self.client = self.session.create_client('lex-runtime', self.region)

    def test_unsigned_payload(self):
        params = {
            'botName': 'foo',
            'botAlias': 'bar',
            'userId': 'baz',
            'contentType': 'application/octet-stream',
            'inputStream': b''
        }

        timestamp = datetime(2017, 3, 22, 0, 0)

        with mock.patch('botocore.auth.datetime') as _datetime:
            _datetime.datetime.utcnow.return_value = timestamp
            with mock.patch('botocore.endpoint.Session.send') as _send:
                _send.return_value = mock.Mock(
                    status_code=200, headers={}, content=b'{}')
                self.client.post_content(**params)
                request = _send.call_args[0][0]

        # The payload gets added to the string to sign, and then part of the
        # signature. The signature will be part of the authorization header.
        # Since we don't have direct access to the payload signature,
        # we compare the authorization instead.
        authorization = request.headers.get('authorization')

        expected_authorization = (
            b'AWS4-HMAC-SHA256 '
            b'Credential=access_key/20170322/us-west-2/lex/aws4_request, '
            b'SignedHeaders=content-type;host;x-amz-content-sha256;x-amz-date,'
            b' Signature='
            b'7f93fde5c36163dce6ee116fcfebab13474ab903782fea04c00bb1dedc3fc4cc'
        )
        self.assertEqual(authorization, expected_authorization)

        content_header = request.headers.get('x-amz-content-sha256')
        self.assertEqual(content_header, b'UNSIGNED-PAYLOAD')
