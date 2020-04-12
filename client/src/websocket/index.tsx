import React, { createContext, useContext } from 'react';
import { WebSocketMessageType, WebSocketMessage } from './messages';

const WebSocketContext = createContext<WebSocket | undefined>(undefined);

interface IProps {
  socket?: WebSocket
  children: React.ReactNode
}

const WebSocketProvider = ({ children, socket }: IProps) =>
  <WebSocketContext.Provider value={socket}>
    {children}
  </WebSocketContext.Provider>;

const useWebSocket = () => {
  const connection = useContext(WebSocketContext);

  if (connection === undefined) {
    throw new Error('useWsSendMessage must be used within a WebSocketProvider');
  }

  return {
    send: (message: WebSocketMessageType) => connection.send(JSON.stringify(message)),
  };
};



export { WebSocketProvider, useWebSocket, WebSocketMessage };
