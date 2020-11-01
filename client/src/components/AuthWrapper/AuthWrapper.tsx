import { Backdrop, CircularProgress } from '@material-ui/core';
import React, { useEffect } from 'react';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { UserStatus } from '../../store/user';
import { getItem, USER_SESSION_UUID_KEY } from '../../utils/storage';
import { useWebSocket, WebSocketMessage } from '../../websocket';
import AuthLayout from '../Layout/AuthLayout';
import Layout from '../Layout/Layout';


const AuthWrapper = () => {
  const userStatus = useStoreState(state => state.user.status);
  const setConnecting = useStoreActions(actions => actions.user.connecting);

  const webSocket = useWebSocket();

  useEffect(() => {
    const existingSession = getItem(USER_SESSION_UUID_KEY);
    if (userStatus === UserStatus.LOGGED_OUT && existingSession) {
      setConnecting();
      webSocket.send(WebSocketMessage.reconnect(existingSession));
    }
  }, [userStatus, setConnecting, webSocket]);

  switch (userStatus) {
    case UserStatus.CONNECTING:
      return <Backdrop open>
        <CircularProgress color="inherit" />
      </Backdrop>;
    case UserStatus.LOGGED_OUT:
      return <AuthLayout/>;
    case UserStatus.LOGGED_IN:
      return <Layout/>;
  }
};

export default AuthWrapper;
