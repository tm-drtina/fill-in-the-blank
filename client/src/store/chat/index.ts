import { Action, action } from 'easy-peasy'

export interface Message {
  username: string,
  timestamp: string,
  content: string,
  systemMsg: boolean,
}

export interface IChatModel {
  global: Message[]
  addGlobalMessage: Action<IChatModel, Message>
  lobby: Message[]
  addLobbyMessage: Action<IChatModel, Message>
  reset: Action<IChatModel>
}

const chat: IChatModel = {
  global: [],
  addGlobalMessage: action((state, value) => {
    state.global.push(value)
  }),
  lobby: [],
  addLobbyMessage: action((state, value) => {
    state.lobby.push(value)
  }),
  reset: action((state) => {
    state.global = []
    state.lobby = []
  }),
}

export default chat
