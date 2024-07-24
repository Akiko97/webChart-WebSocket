from django.urls import path
from wsServerApp import consumers

websocket_urlpatterns = [
    path('ws', consumers.MyConsumer.as_asgi()),
]
