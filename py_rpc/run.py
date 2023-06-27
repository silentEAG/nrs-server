from concurrent import futures

import grpc

from src.model import Config
from src.proto import NewsRecommend, data_pb2_grpc

config = Config()

def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    data_pb2_grpc.add_NewsRecommendServicer_to_server(NewsRecommend(), server)
    server.add_insecure_port(f'{config.host}:{config.port}')
    server.start()
    print(f'Server started at {config.host}:{config.port}')
    server.wait_for_termination()

if __name__ == '__main__':
    serve()