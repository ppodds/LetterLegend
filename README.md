# Letter Legend

Letter Legend is a multi-player game. Players take turns to spell words using the letters which are available to them. The game ends when reach the maximum number of rounds. The player with the highest score wins.

## Features

- Multi-player
- Real-time

## Game Protocol

- Request
    - Header (12 bytes)
        - OP code
            - 1 byte
        - Reserved
            - 3 bytes
        - State
            - 0~1: reserved, others: random
            - same as the state in response
            - 4 bytes 
        - Content length
            - 4 bytes
    - Body
        - Content
            - Content length bytes
- Response
    - Header (8 byte)
        - State
            - 0: lobby broadcast
            - 1: game broadcast
            - others: same as the state in request
            - 4 bytes 
        - Content length
            - 4 bytes
    - Body
        - Content
            - Content length bytes

The state can be used to identify the type of the response. For example, if the state is 0, the response is a lobby broadcast. If the state is 1, the response is a game broadcast. If the state is others, the response is a response of request.
