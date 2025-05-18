interface IAuthContructorProps {
  login: string;
  password?: string;
  access: string;
  refresh: string;    
}

interface IUserContructorProps {
  id: string;
  email: string;
  username: string;
  displayname: string;
  tag: string;
  avatarFilename?: string;
  bannerFilename?: string;
  bannerColor?: string;
  roles: string[];
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

export class User {
  public id: string;
  public email: string;
  public username: string;
  public displayname: string;
  public tag: string;
  public avatarFilename?: string;
  public bannerFilename?: string;
  public bannerColor?: string;
  public roles: string[];
  
  constructor({ id, email, username, displayname, tag, avatarFilename, bannerFilename, bannerColor, roles }: IUserContructorProps) {
    this.id = id;
    this.email = email;
    this.username = username;
    this.displayname = displayname;
    this.tag = tag;
    this.avatarFilename = avatarFilename;
    this.bannerFilename = bannerFilename;
    this.bannerColor = bannerColor;
    this.roles = roles;
  }
}