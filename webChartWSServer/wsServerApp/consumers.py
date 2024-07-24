import asyncio
import random
from channels.generic.websocket import AsyncWebsocketConsumer
import logging

logger = logging.getLogger(__name__)

class MyConsumer(AsyncWebsocketConsumer):
    async def connect(self):
        await self.accept()
        self.send_task = asyncio.create_task(self.send_random_number())
        logger.info("WebSocket connection established")

    async def disconnect(self, close_code):
        self.send_task.cancel()
        logger.info("WebSocket connection closed")

    async def receive(self, text_data=None, bytes_data=None):
        if text_data:
            logger.debug(f"Received message: {text_data}")
            await self.send(text_data=text_data)
        elif bytes_data:
            logger.warn("Received binary message")

    async def send_random_number(self):
        while True:
            await asyncio.sleep(5)
            random_number = random.randint(0, 100)
            logger.debug(f"Sending random number: {random_number}")
            await self.send(text_data=f"{random_number}")
