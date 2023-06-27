from pydantic import BaseSettings


class Config(BaseSettings):
    port: int = 50051
    host: str = 'localhost'
 
    class Config:
        env_file = '.env'
        env_file_encoding = 'utf-8'