# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from aptos.datastream.v1 import datastream_pb2 as aptos_dot_datastream_dot_v1_dot_datastream__pb2


class IndexerStreamStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.RawDatastream = channel.unary_stream(
                '/aptos.datastream.v1.IndexerStream/RawDatastream',
                request_serializer=aptos_dot_datastream_dot_v1_dot_datastream__pb2.RawDatastreamRequest.SerializeToString,
                response_deserializer=aptos_dot_datastream_dot_v1_dot_datastream__pb2.RawDatastreamResponse.FromString,
                )


class IndexerStreamServicer(object):
    """Missing associated documentation comment in .proto file."""

    def RawDatastream(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_IndexerStreamServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'RawDatastream': grpc.unary_stream_rpc_method_handler(
                    servicer.RawDatastream,
                    request_deserializer=aptos_dot_datastream_dot_v1_dot_datastream__pb2.RawDatastreamRequest.FromString,
                    response_serializer=aptos_dot_datastream_dot_v1_dot_datastream__pb2.RawDatastreamResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'aptos.datastream.v1.IndexerStream', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class IndexerStream(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def RawDatastream(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_stream(request, target, '/aptos.datastream.v1.IndexerStream/RawDatastream',
            aptos_dot_datastream_dot_v1_dot_datastream__pb2.RawDatastreamRequest.SerializeToString,
            aptos_dot_datastream_dot_v1_dot_datastream__pb2.RawDatastreamResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
