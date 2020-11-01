import { Backdrop, Button, CircularProgress, TextField } from '@material-ui/core';
import { useSnackbar } from 'notistack';
import React, { useCallback, useEffect, useState } from 'react';
import { useHistory } from 'react-router';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { LobbyStatus } from '../../store/lobby';
import { useWebSocket, WebSocketMessage } from '../../websocket';
import Dialog from '../Dialog/Dialog';


const CreateLobby = () => {
  const setLobbyStatus = useStoreActions(actions => actions.lobby.setStatus);
  const resetLobby = useStoreActions(actions => actions.lobby.reset);
  const lobbyStatus = useStoreState(state => state.lobby.status);
  const failureReason = useStoreState(state => state.lobby.failureReason);
  const detail = useStoreState(state => state.lobby.detail);

  const webSocket = useWebSocket();
  const { enqueueSnackbar } = useSnackbar();
  const { push } = useHistory();

  const [modalOpen, setModalOpen] = useState(false);
  const [lobbyName, setLobbyName] = useState('');
  const [error, setError] = useState<string | undefined>();

  const onOpen = useCallback(() => {
    setLobbyName('');
    setError(undefined);
    setModalOpen(true);
  }, [setModalOpen, setLobbyName, setError]);

  const onClose = useCallback(() => {
    setModalOpen(false);
  }, [setModalOpen]);

  const onSubmit = useCallback(event => {
    event.preventDefault();
    if (lobbyName.trim().length === 0) {
      setError('Lobby name cannot be empty');
      return;
    }
    webSocket.send(WebSocketMessage.createLobby(lobbyName));

    setModalOpen(false);
    setLobbyName('');
    setLobbyStatus(LobbyStatus.JOINING);
  }, [setModalOpen, setLobbyName, webSocket, lobbyName, setLobbyStatus]);

  const onChange = useCallback(event => {
    if (event.target.value.trim().length === 0) {
      setError('Lobby name cannot be empty');
    } else {
      setError(undefined);
    }
    setLobbyName(event.target.value);
  }, [setLobbyName, setError]);

  useEffect(() => {
    if (lobbyStatus === LobbyStatus.FAILED) {
      enqueueSnackbar(failureReason, { variant: 'error' });
      resetLobby();
    }
    if (lobbyStatus === LobbyStatus.JOINED && detail !== undefined) {
      push(`/lobby/${detail.lobby_id}`)
    }
  }, [lobbyStatus, resetLobby, enqueueSnackbar, failureReason, push, detail]);

  return (
    <>
      <Button onClick={onOpen}>Create lobby</Button>
      <Dialog open={modalOpen} onClose={onClose}>
        <form onSubmit={onSubmit}>
          <TextField
            error={error !== undefined}
            helperText={error}
            value={lobbyName}
            onChange={onChange}
            label="Lobby name"
          />
          <br/>
          <Button type="submit">Create</Button>
          <Button type="reset" onClick={onClose}>Close</Button>
        </form>
      </Dialog>
      <Backdrop open={lobbyStatus === LobbyStatus.JOINING}>
        <CircularProgress color="inherit"/>
      </Backdrop>
    </>
  );
};

export default CreateLobby;
