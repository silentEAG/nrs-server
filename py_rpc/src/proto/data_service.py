from . import data_pb2, data_pb2_grpc

class NewsRecommend(data_pb2_grpc.NewsRecommendServicer):
    def GetRecommendations(self, request, context):
        # TODO: implement
        ...
        return data_pb2.GetRecommendationsResponse(...)
    