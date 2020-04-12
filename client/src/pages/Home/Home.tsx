import { Box } from '@material-ui/core';
import React, { useState } from 'react';

import Wrapper from '../../components/Wrapper/Wrapper';
import { useStoreState } from '../../store/hooks';
import { useWebSocket, WebSocketMessage } from '../../websocket';


const Home: React.FC = () => {
  const globalMessages = useStoreState(state => state.chat.global);
  const [formInput, formInputChange] = useState("")
  const webSocket = useWebSocket();
  const onSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    webSocket.send(WebSocketMessage.globalChat(formInput));
    event.preventDefault();
  }

  return (
    <>
      <Wrapper>
        <Box display="flex">
          <Box mt={12} display="flex" flexDirection="column" alignItems="flex-start" width={0.5}>
            <ul>
              {globalMessages.map((message, index) =>
                <li key={index}>{message.timestamp} <strong>{message.username}:</strong> {message.content}</li>
              )}
            </ul>
            <form onSubmit={onSubmit}>
              <input type="text" value={formInput} onChange={event => formInputChange(event.target.value)}/>
              <input type="submit"/>
            </form>
            text
          </Box>
        </Box>
      </Wrapper>
    </>
  );
};

export default Home;
