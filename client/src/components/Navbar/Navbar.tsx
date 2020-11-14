import { Box } from '@material-ui/core';
import AppBar from '@material-ui/core/AppBar';
import Button from '@material-ui/core/Button';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import React from 'react';
import { useHistory } from 'react-router-dom';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { removeItem, USER_SESSION_UUID_KEY } from '../../utils/storage';
import { useWebSocket, WebSocketMessage } from '../../websocket';

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      flexGrow: 1,
    },
    title: {
      cursor: 'pointer',
    },
  })
);

const Navbar = () => {
  const { push } = useHistory();
  const classes = useStyles();
  const webSocket = useWebSocket();
  const username = useStoreState(state => state.user.username)
  const logout = useStoreActions(actions => actions.user.logout)

  const onLogout = async () => {
    removeItem(USER_SESSION_UUID_KEY);
    logout();
    webSocket.send(WebSocketMessage.logout());
  }

  return (
    <>
      <div className={classes.root}>
        <AppBar position="static" color="transparent">
          <Toolbar>
            <Typography onClick={() => push('/')} variant="h6" className={classes.title}>
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
      </div>
    </>
  );
};

export default Navbar;
