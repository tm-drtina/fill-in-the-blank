import React, { useEffect, useState } from 'react';
import config from '../../config';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { UserStatus } from '../../store/user';
import { WebSocketStatus } from '../../store/websocket';
import { getItem, USER_SESSION_UUID_KEY } from '../../utils/storage';
import { WebSocketMessage, WebSocketProvider } from '../../websocket';
import AuthWrapper from '../AuthWrapper/AuthWrapper';

const WebSocketWrapper = () => {
  const setSocketStatus = useStoreActions(actions => actions.websocket.setStatus);
  const onMessage = useStoreActions(actions => actions.websocket.onMessage);
  const setUserStatus = useStoreActions(actions => actions.user.setStatus);
  const socketStatus = useStoreState(state => state.websocket.status);

  const [ws, setWS] = useState<WebSocket>();

  useEffect(() => {
    setWS(new WebSocket(config.wsPath))
  }, [])

  useEffect(() => {
    if (!ws) return
    ws.onopen = () => {
      setSocketStatus(WebSocketStatus.CONNECTED);
      const existingSession = getItem(USER_SESSION_UUID_KEY);
      if (existingSession) {
        setUserStatus(UserStatus.RECONNECTING);
        ws.send(JSON.stringify(WebSocketMessage.reconnect(existingSession)));
      } else {
        setUserStatus(UserStatus.LOGGED_OUT);
      }
    };
    ws.onmessage = (e) => onMessage(e.data)
    ws.onerror = (e) => {
      console.error('Socket encountered error: ', e, 'Closing socket');
      ws.close();
    };
    ws.onclose = () => {
      setSocketStatus(WebSocketStatus.CLOSED);
      setUserStatus(UserStatus.LOGGED_OUT);
      setTimeout(() => {
        setSocketStatus(WebSocketStatus.CONNECTING);
        setWS(new WebSocket(config.wsPath));
      }, 2_000);
    };
  }, [ws, setSocketStatus, setUserStatus, onMessage]);

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
