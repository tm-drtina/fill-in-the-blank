import { Backdrop, Button, CircularProgress } from '@material-ui/core';
import { useSnackbar } from 'notistack';
import React, { useCallback, useEffect } from 'react';
import { useHistory, useParams } from 'react-router';
import { Link as RouterLink } from 'react-router-dom';
import Chat from '../../components/Chat/Chat';

import Wrapper from '../../components/Wrapper/Wrapper';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { LobbyStatus } from '../../store/lobby';
import { useWebSocket, WebSocketMessage } from '../../websocket';

type TParams =  { id: string };

const Lobby: React.FC = () => {
  const messages = useStoreState(state => state.chat.lobby);
  const status = useStoreState(state => state.lobby.status);
  const failureReason = useStoreState(state => state.lobby.failureReason);
  const setStatus = useStoreActions(actions => actions.lobby.setStatus);
  const resetLobby = useStoreActions(actions => actions.lobby.reset);
  const detail = useStoreState(state => state.lobby.detail);

  const webSocket = useWebSocket();
  const { enqueueSnackbar } = useSnackbar();
  const { id } = useParams<TParams>();
  const { push } = useHistory();

  useEffect(() => {
    if (status === LobbyStatus.NO_LOBBY) {
      setStatus(LobbyStatus.JOINING);
      webSocket.send(WebSocketMessage.joinLobby(id));
    }
  }, [id, webSocket, status, setStatus]);

  useEffect(() => {
    if (status === LobbyStatus.FAILED) {
      enqueueSnackbar(failureReason, { variant: 'error' });
      resetLobby();
      push('/');
    }
  }, [status, failureReason, enqueueSnackbar, push, resetLobby]);

  const sendMessage = useCallback(msg => {
    webSocket.send(WebSocketMessage.lobbyChat(msg));
  }, [webSocket]);

  const leaveLobby = useCallback(() => {
    webSocket.send(WebSocketMessage.leaveLobby());
    setStatus(LobbyStatus.NO_LOBBY);
  }, [webSocket, setStatus]);

  return (
    <>
      <Wrapper>
        {status === LobbyStatus.JOINED && <>
            <h2>Lobby: {detail?.name}</h2>
            <h4>Players:</h4>
            <ul>
              {detail?.players.map((player, index) => <>
                <li key={index}>{player}</li>
              </>)}
            </ul>
            <Button onClick={leaveLobby} color="primary" component={RouterLink} to="/">
                Leave
            </Button>
            <Chat messages={messages} sendMessage={sendMessage}/>
        </>}
        {status !== LobbyStatus.JOINED && <>
            <Backdrop open>
                <CircularProgress color="inherit"/>
            </Backdrop>
        </>}
      </Wrapper>
    </>
  );
};

export default Lobby;
