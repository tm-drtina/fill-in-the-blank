import React, { useEffect, useState } from 'react';
import config from '../../config';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { WebSocketStatus } from '../../store/websocket';
import { WebSocketProvider } from '../../websocket';
import AuthWrapper from '../AuthWrapper/AuthWrapper';

const WebSocketWrapper = () => {
  const setSocketStatus = useStoreActions(actions => actions.websocket.setStatus);
  const onMessage = useStoreActions(actions => actions.websocket.onMessage);
  const userLogout = useStoreActions(actions => actions.user.logout);
  const chatReset = useStoreActions(actions => actions.chat.reset);
  const lobbyReset = useStoreActions(actions => actions.lobby.reset);
  const socketStatus = useStoreState(state => state.websocket.status);

  const [ws, setWS] = useState<WebSocket>();

  useEffect(() => {
    setWS(new WebSocket(config.wsPath));
  }, []);

  useEffect(() => {
    if (!ws) return;
    ws.onopen = () => {
      setSocketStatus(WebSocketStatus.CONNECTED);
    };
    ws.onmessage = (e) => onMessage(e.data);
    ws.onerror = (e) => {
      console.error('Socket encountered error: ', e, 'Closing socket');
      ws.close();
    };
    ws.onclose = () => {
      setSocketStatus(WebSocketStatus.CLOSED);
      userLogout();
      chatReset();
      lobbyReset();
      setTimeout(() => {
        setSocketStatus(WebSocketStatus.CONNECTING);
        setWS(new WebSocket(config.wsPath));
      }, 2_000);
    };
  }, [ws, setSocketStatus, userLogout, chatReset, lobbyReset, onMessage]);

  switch (socketStatus) {
    case WebSocketStatus.CLOSED:
      return <div>Lost connection to the server</div>;
    case WebSocketStatus.CONNECTING:
      return <div>Connecting to the server</div>;
    case WebSocketStatus.CONNECTED:
      return <WebSocketProvider socket={ws}>
        <AuthWrapper/>
      </WebSocketProvider>;
  }
};

export default WebSocketWrapper;
