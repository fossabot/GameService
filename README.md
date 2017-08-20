## Why?
It would be beneficial to handle the games session progress with a micro-service, to offset some of the work that has to be done in the main session, this also allows miki being able to be restarted and the current progress of the game to not be lost (assuming the micro service was not stopped) this does add a bit of complexity, however it adds a bit of safety to add a micro-service and could allow more flexibility

## Design
- The micro service should focus heavily on saftey and ease of communication.
- The microservice should hanlde the tasks its meant to take care of, and only return the sponse