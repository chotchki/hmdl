import React, { useCallback, useContext, useState } from 'react';
import PropTypes from 'prop-types';

// We only provide the current role, haven't figured out a need for your username
const AuthenticationContext = React.createContext(null);

const AuthenticationProvider = ({ children }) => {
  const [role, setAuthRole] = useState('Anonymous');

  const isAdmin = useCallback(() => {
    return role === 'Admin';
  }, [role]);

  const isAnonymous = useCallback(() => {
    return role === 'Anonymous';
  }, [role]);

  const isRegistered = useCallback(() => {
    return role === 'Registered';
  }, [role]);

  const setRole = useCallback((newRole) => {
    setAuthRole(newRole);
  }, []);

  return (
    <AuthenticationContext.Provider value={{ isAdmin, isAnonymous, isRegistered, setRole }}>
      {children}
    </AuthenticationContext.Provider>
  );
};

AuthenticationProvider.propTypes = {
  children: PropTypes.element,
};

const useAuthentication = () => {
  const authHelpers = useContext(AuthenticationContext);

  return authHelpers;
};

export { AuthenticationContext, useAuthentication };
export default AuthenticationProvider;
