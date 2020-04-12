const dev = {
  wsPath: "ws://localhost:8080/ws"
}

const l = window.location;
const prod = {
  wsPath: ((l.protocol === "https:") ? "wss://" : "ws://") + l.host + l.pathname + "ws"
}

export default process.env.NODE_ENV === 'development' ? dev : prod;
