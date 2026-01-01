import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../environments/environment';
import {LoginService} from '../login.service';
import {UserPageInfo} from './user-service';

export interface RolePageInfo{
  page: number;
  perSize: number;
  total_items: number;
  total_pages: number;
  items: Role[];
}
export interface Role{
  id: number;
  name: string;
  email: string;
  role: string;
  last_modified_by: string;
  last_modified_on: Date;
}
export interface RolePermissionPageInfo{
  page: number;
  perSize: number;
  total_items: number;
  total_pages: number;
  items: RolePermission[];
}
export interface RolePermission{
  id: number;
  name: string;
  email: string;
  role: string;
  last_modified_by: string;
  last_modified_on: Date;
}
@Injectable({
  providedIn: 'root',
})
export class RolesAndPermissionsService {
  private apiUrl = environment.apiUrl;
  constructor(private http: HttpClient, private LoginService:LoginService) {
    console.log('RolesAndPermissionsService Constructing');
  }

  getRoles():Observable<RolePageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<RolePageInfo>(`${this.apiUrl}/api/roles`, {headers});

  }
  getRolePermissions(): Observable<RolePermissionPageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<RolePermissionPageInfo>(`${this.apiUrl}/api/role_permissions`, {headers});

  }
}
