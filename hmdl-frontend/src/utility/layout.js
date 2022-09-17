import React from 'react';
import { Outlet } from 'react-router-dom';

import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import NavDropdown from 'react-bootstrap/NavDropdown';
import Nav from 'react-bootstrap/Nav';
import { LinkContainer } from 'react-router-bootstrap';
import { useAuthentication } from './authentication/AuthenticationProvider';

export function Layout() {
  const { isAdmin } = useAuthentication();

  return (
    <Container fluid>
      <Navbar bg="primary" variant="dark">
        <Container>
          <Navbar.Brand>HMDL</Navbar.Brand>
          <Navbar.Toggle aria-controls="basic-navbar-nav" />
          <Navbar.Collapse id="basic-navbar-nav">
            <Nav className="me-auto">
              {isAdmin &&
                <>
                  <NavDropdown title="Domains">
                    <LinkContainer to="/domains">
                      <NavDropdown.Item url="/domains">Uncategorized Domains</NavDropdown.Item>
                    </LinkContainer>
                    <LinkContainer to="/domain-groups">
                      <NavDropdown.Item url="/domain-groups">Domain Groups</NavDropdown.Item>
                    </LinkContainer>
                  </NavDropdown>
                  <NavDropdown title="Clients">
                    <LinkContainer to="/clients">
                      <NavDropdown.Item url="/clients">Uncategorized Clients</NavDropdown.Item>
                    </LinkContainer>
                    <LinkContainer to="/client-groups">
                      <NavDropdown.Item url="/client-groups">Client Groups</NavDropdown.Item>
                    </LinkContainer>
                  </NavDropdown>
                </>
              }
            </Nav>
          </Navbar.Collapse>
        </Container>
      </Navbar>
      <Container>
        <Outlet />
      </Container>
    </Container>
  );
}

export default Layout;
