import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {environment} from '../environments/environment';
import {Observable} from 'rxjs';


export interface LoginRequest{
  email: string;
  password: string;
}

export interface LoginResponse{
  user_id: number;
  name: string;
  email: string;
  role_id: number;
  role_name: string;
  token: string;
}
@Injectable({
  providedIn: 'root',
})
export class LoginService {
  private apiUrl = environment.apiUrl;
  IsLoggedIn: boolean = false;
  constructor( private httpClient: HttpClient ) {}
  login(email:string, password:string):Observable<LoginResponse>{
    const body: LoginRequest = {email, password};
    return this.httpClient.post<LoginResponse>(`${this.apiUrl}/login`, body);
  }
}
