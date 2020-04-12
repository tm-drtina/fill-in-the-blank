import React, { useCallback } from 'react';
import { useWebSocket, WebSocketMessage } from '../../websocket';

const AuthLayout: React.FC = () => {
  const webSocket = useWebSocket();

  const onSubmit = useCallback((e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const { username } = e.target as any;
    webSocket.send(WebSocketMessage.connect(username.value));
  }, [webSocket])

  return (
    <>
      <form action="#" onSubmit={onSubmit}>
        <input type="text" id="username"/>
        <input type="submit"/>
      </form>
    </>
  );
};

export default AuthLayout;
