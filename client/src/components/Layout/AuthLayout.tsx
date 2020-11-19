import { Box, Button, Container, FormControl, InputAdornment, Paper, TextField } from '@material-ui/core';
import React, { useCallback, useState } from 'react';
import { useStoreActions, useStoreState } from '../../store/hooks';
import { useWebSocket, WebSocketMessage } from '../../websocket';
import { AccountCircle } from "@material-ui/icons";
import { makeStyles } from "@material-ui/core/styles";
import { Alert } from "@material-ui/lab";

const AuthLayout: React.FC = () => {
    const lastError = useStoreState(state => state.user.lastError);
    const setConnecting = useStoreActions(actions => actions.user.connecting);

    const webSocket = useWebSocket();

    const [username, setUsername] = useState("");
    const [usernameError, setUsernameError] = useState<undefined | string>(undefined);

    const onSubmit = useCallback((e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (validateUsername(username)) {
            setConnecting();
            webSocket.send(WebSocketMessage.connect(username));
        }
    }, [webSocket, setConnecting, username]);

    const validateUsername = (username: string) => {
        console.log(username)
        if (username.length < 3) {
            setUsernameError("Username must be at least 3 characters long");
            return false;
        }

        setUsernameError(undefined);
        return true;
    }

    const onChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
        const newName = e.target.value.trim();
        validateUsername(newName)
        setUsername(newName);
    }, [setUsername])

    const useStyles = makeStyles(() => ({
        root: {
            display: 'flex',
            flexWrap: 'wrap'
        },
        center: {
            alignItems: 'center'
        }
    }));
    const classes = useStyles();

    return (
        <Container maxWidth="sm">
            <Box mt={2}>
                {lastError &&
                <Box mb={2}>
                    <Alert severity="error">{lastError}</Alert>
                </Box>
                }
                <Paper elevation={3}>
                    <Box padding={2}>
                        <form action="#" onSubmit={onSubmit}>
                            <div className={classes.root}>
                                <FormControl fullWidth>
                                    <TextField label="Username" value={username} onChange={onChange}
                                               error={usernameError !== undefined}
                                               helperText={usernameError}
                                               InputProps={{
                                                   startAdornment: (
                                                       <InputAdornment position="start">
                                                           <AccountCircle/>
                                                       </InputAdornment>
                                                   ),
                                               }}
                                    />
                                </FormControl>
                                <FormControl fullWidth className={classes.center}>
                                    <Box mt={2}>
                                        <Button type="submit" color="primary" variant="contained"
                                                size="small">Login</Button>
                                    </Box>
                                </FormControl>
                            </div>
                        </form>
                    </Box>
                </Paper>
            </Box>
        </Container>

    );
};

export default AuthLayout;
