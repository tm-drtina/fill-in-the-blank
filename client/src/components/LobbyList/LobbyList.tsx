import { Box, Button } from '@material-ui/core';
import React, { useEffect } from 'react';
import { Link as RouterLink } from 'react-router-dom';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { useWebSocket, WebSocketMessage } from '../../websocket';
import Wrapper from '../Wrapper/Wrapper';

const LobbyList: React.FC = () => {
  const lobbyList = useStoreState(state => state.lobby.list);
  const lobbyReset = useStoreActions(actions => actions.lobby.reset);
  const webSocket = useWebSocket();

  useEffect(() => {
    lobbyReset();
  }, [lobbyReset]);

  useEffect(() => {
    if (lobbyList === undefined) {
      webSocket.send(WebSocketMessage.listLobbies());
    }
  }, [lobbyList, webSocket]);

  return (
    <Wrapper>
      <Box display="flex">
        <Box mt={12} display="flex" flexDirection="column" alignItems="flex-start" width={0.5}>
          <>
            <h3>List of lobbies:</h3>
            {lobbyList === undefined && 'Loading lobbies'}
            {lobbyList !== undefined && lobbyList.map(lobby =>
              <React.Fragment key={lobby.lobby_id}>
                Lobby: {lobby.name}
                <br/>
                Players: {lobby.player_count}
                <br/>
                <Button component={RouterLink} to={`/lobby/${lobby.lobby_id}`}>
                  Join
                </Button>
              </React.Fragment>
            )}
            {lobbyList !== undefined && lobbyList.length === 0 && (<>
              No lobbies.
            </>)}
          </>
        </Box>
      </Box>
    </Wrapper>
  );
};

export default LobbyList;
