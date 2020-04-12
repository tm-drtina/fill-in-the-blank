import { createStore } from 'easy-peasy';

import chat from './chat';
import user from './user';
import websocket from './websocket';

export const storeModel = {
  chat,
  user,
  websocket,
};

export default createStore(storeModel);
