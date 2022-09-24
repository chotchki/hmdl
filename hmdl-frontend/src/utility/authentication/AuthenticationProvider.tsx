import React, { createContext, useCallback, useContext, useState } from 'react';
import PropTypes from 'prop-types';

// We only provide the current role, haven't figured out a need for your username
interface RoleContextProps {
  role: string;
  isAdmin: () => boolean;
  isAnonymous: () => boolean;
  isRegistered: () => boolean;
  setRole: (r: string) => void;
}

const RoleContext = createContext<RoleContextProps>({
  role: 'Anonymous',
  isAdmin: () => { console.log('RoleContext wrong'); return false; },
  isAnonymous: () => { console.log('RoleContext wrong'); return false; },
  isRegistered: () => { console.log('RoleContext wrong'); return false; },
  setRole: (r: string) => { console.log('RoleContext wrong'); },
});

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
    <RoleContext.Provider value={{ role, isAdmin, isAnonymous, isRegistered, setRole }}>
      {children}
    </RoleContext.Provider>
  );
};

AuthenticationProvider.propTypes = {
  children: PropTypes.element,
};

const useAuthentication = () => {
  const authHelpers = useContext(RoleContext);

  return authHelpers;
};

export { RoleContext, useAuthentication };
export default AuthenticationProvider;
