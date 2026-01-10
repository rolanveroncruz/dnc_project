import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../environments/environment';
import {LoginService} from '../login.service';

export interface UserPageInfo{
  page: number;
  perSize: number;
  total_items: number;
  total_pages: number;
  items: User[];
}
export interface User{
  id: number;
  name: string;
  email: string;
  role_id: number;
  role: string;
  last_modified_by: string;
  last_modified_on: Date;
}
export interface NewOrPatchUser{
  name: string;
  password?: string;
  email: string;
  role_id: number;
}
@Injectable({
  providedIn: 'root',
})
export class UserService {
  private apiUrl = environment.apiUrl;
  constructor(private http: HttpClient, private LoginService:LoginService) {}

  getUsers():Observable<UserPageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<UserPageInfo>(`${this.apiUrl}/api/users?`, {headers});
  }
  postUser(newUser:NewOrPatchUser){
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    console.log("In User Service, Post User:", newUser);
    return this.http.post<User>(`${this.apiUrl}/api/users/`, newUser, {headers});
  }
  patchUser(userId:number, newUser:NewOrPatchUser){
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.patch<User>(`${this.apiUrl}/api/users/${userId}`, newUser, {headers});
  }

}
