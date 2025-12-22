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
}
@Injectable({
  providedIn: 'root',
})
export class LoginService {
  private apiUrl = environment.apiUrl;
  currentUser: LoginResponse | undefined;
  IsLoggedIn: boolean = false;
  constructor( private httpClient: HttpClient ) {}

  login(email:string, password:string):Observable<LoginResponse>{
    const body: LoginRequest = {email, password};
    return this.httpClient.post<LoginResponse>(`${this.apiUrl}/login`, body).pipe(
      tap( response=> {
        if (this.isValid(response)){
          this.setUser(response);
        }else{
          console.log("In Service, Login Failed");
        }
      })
    )
  }
  // Check if the response is valid. If so, write it to the local storage.
  isValid(user: LoginResponse):boolean {


    return true;
  }

  setUser(user: LoginResponse){
    this.currentUser = user;
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
