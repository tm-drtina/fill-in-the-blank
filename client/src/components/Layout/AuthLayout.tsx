import { Button, TextField } from '@mui/material';
import React, { useCallback, useState } from 'react';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { useWebSocket, WebSocketMessage } from '../../websocket';

const AuthLayout: React.FC = () => {
  const lastError = useStoreState(state => state.user.lastError);
  const setConnecting = useStoreActions(actions => actions.user.connecting);

  const webSocket = useWebSocket();

  const [username, setUsername] = useState("");
  const [usernameError, setUsernameError] = useState<undefined | string>(undefined);

  const onSubmit = useCallback((e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setConnecting();
    webSocket.send(WebSocketMessage.connect(username));
  }, [webSocket, setConnecting, username]);

  const onChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.value.trim().length < 3) {
      setUsernameError("Username must be at least 3 characters long");
    } else {
      setUsernameError(undefined);
    }
    setUsername(e.target.value);
  }, [setUsername, setUsernameError])

  return (
    <>
      <form action="#" onSubmit={onSubmit}>
        <TextField
          value={username}
          onChange={onChange}
          label="Username"
          error={usernameError !== undefined || lastError !== undefined}
          helperText={usernameError || lastError}
        />

        <br/>
        <Button type="submit">Login</Button>
      </form>

      {lastError && <>
          <div>{lastError}</div>
      </>}
    </>
  );
};

export default AuthLayout;
