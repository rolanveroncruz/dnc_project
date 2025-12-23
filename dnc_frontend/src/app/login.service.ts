import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {environment} from '../environments/environment';
import {Observable, tap} from 'rxjs';


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
  menu_activation_map: Record<string, string>;
}
@Injectable({
  providedIn: 'root',
})
export class LoginService {
  private apiUrl = environment.apiUrl;
  currentUser: LoginResponse | undefined;
  IsLoggedIn: boolean = false;
  menu_activation_map: Map<string, string> |  undefined;
  constructor( private httpClient: HttpClient ) {
    console.log('Login Service Initialized', Math.random())
  }

  login(email:string, password:string):Observable<LoginResponse>{
    const body: LoginRequest = {email, password};
    return this.httpClient.post<LoginResponse>(`${this.apiUrl}/login`, body).pipe(
      tap( response=> {
        let isValid = this.isValid(response);
        if (isValid){
          this.menu_activation_map = new Map(Object.entries(response.menu_activation_map))
          this.setUser(response);
        }else{
          console.log("In Service, Login Failed");
        }
      })
    )
  }
  getMenuActivationMap():Map<string, string> | undefined{
    return this.menu_activation_map;
  }

  // Check if the response is valid. If so, write it to the local storage.
  isValid(x: any):boolean {
    return (
      x &&
        typeof x === 'object' &&
        typeof x.user_id === 'number' &&
        x.user_id > 0 &&
        typeof x.role_id ===  'number' &&
        x.role_id > 0 &&
        typeof x.name === 'string' &&
        x.name.trim().length > 0 &&
        typeof x.email === 'string' &&
        x.email.includes('@') &&
        typeof x.role_name === 'string' &&
        x.role_name.trim().length > 0 &&
        looksLikeJwt(x.token) &&
        isRecordOfStrings(x.menu_activation_map)
    );
  }

  setUser(user: LoginResponse){
    this.currentUser = user;
    this.IsLoggedIn = true;
  console.log("In Service, Login Success:", this.currentUser);
  }
  logout(){
    this.currentUser = undefined;
    this.IsLoggedIn = false;
  }
  isLoggedIn(){
    return this.IsLoggedIn;
  }

}

function looksLikeJwt(token: unknown): boolean{
  if (typeof token !== 'string') return false;
  const parts = token.split('.');
  return parts.length === 3 && parts.every(p=>p.length > 0);
}
function isRecordOfStrings(x:unknown): boolean{
  if (!x || typeof x !== 'object' || Array.isArray(x)) return false;
  return Object.values( x as Record<string, unknown>).every(v=>typeof v === 'string');

}
