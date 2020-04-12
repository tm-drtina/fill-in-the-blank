import React from 'react';
import { useStoreState } from '../../store/hooks';
import { UserStatus } from '../../store/user';
import AuthLayout from '../Layout/AuthLayout';
import Layout from '../Layout/Layout';


const AuthWrapper = () => {
  const userStatus = useStoreState(state => state.user.status);

  switch (userStatus) {
    case UserStatus.RECONNECTING:
      return <div>Reconnecting to previous session.</div>;
    case UserStatus.LOGGED_OUT:
      return <AuthLayout/>;
    case UserStatus.LOGGED_IN:
      return <Layout/>;
  }
};

export default AuthWrapper;
