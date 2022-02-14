import React from 'react';
import Chat from '../../components/Chat/Chat';
import CreateLobby from '../../components/CreateLobby/CreateLobby';
import LobbyList from '../../components/LobbyList/LobbyList';

import Wrapper from '../../components/Wrapper/Wrapper';
import { useStoreState } from '../../store/hooks';
import { useWebSocket, WebSocketMessage } from '../../websocket';


const Home: React.FC = () => {
  const globalMessages = useStoreState(state => state.chat.global);
  const webSocket = useWebSocket();

  return (
    <Wrapper>
      <LobbyList/>
      <CreateLobby/>
      <Chat messages={globalMessages} sendMessage={msg => webSocket.send(WebSocketMessage.globalChat(msg))}/>
    </Wrapper>
  );
};

export default Home;
