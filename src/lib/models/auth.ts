export interface IRegisterRequest {
  email: string;
  displayname: string;
  username: string;
  password: string;
}

export interface ILoginRequest {
  login: string;
  password?: string;
}

export interface IAuthResponse {
  access: string;
  refresh: string;   
}

interface IAuthContructorProps {
  login: string;
  password?: string;
  access: string;
  refresh: string;    
}

export class Auth {
  public login: string;
  public password?: string;
  public access: string;
  public refresh: string;
  
  constructor({ login, password, access, refresh }: IAuthContructorProps) {
    this.login = login;
    this.password = password;
    this.access = access;
    this.refresh = refresh;
  }
}
