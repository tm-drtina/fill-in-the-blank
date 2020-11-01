import { createStore } from 'easy-peasy';

import chat from './chat';
import lobby from './lobby';
import user from './user';
import websocket from './websocket';

export const storeModel = {
  chat,
  lobby,
  user,
  websocket,
};

export default createStore(storeModel);
