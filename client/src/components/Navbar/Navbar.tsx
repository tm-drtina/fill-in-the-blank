import { AppBar, Box, Button, Toolbar, Typography } from '@mui/material';
import { useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { removeItem, USER_SESSION_UUID_KEY } from '../../utils/storage';
import { useWebSocket, WebSocketMessage } from '../../websocket';

const Navbar = () => {
  const navigate = useNavigate();
  const webSocket = useWebSocket();
  const username = useStoreState(state => state.user.username)
  const logout = useStoreActions(actions => actions.user.logout)

  const onLogout = async () => {
    removeItem(USER_SESSION_UUID_KEY);
    logout();
    webSocket.send(WebSocketMessage.logout());
  }

  const toHome = useCallback(() => {
    navigate('/');
  }, [navigate])

  return (
    <AppBar position="static" color="transparent">
      <Toolbar>
        <Typography onClick={toHome} variant="h6" style={{ flexGrow: 1, cursor: 'pointer' }}>
          Fill in the blank
        </Typography>
        <Typography>{username}</Typography>
        <Box ml={3}>
          <Button color="inherit" onClick={onLogout}>
            Log out
          </Button>
        </Box>
      </Toolbar>
    </AppBar>
  );
};

export default Navbar;
