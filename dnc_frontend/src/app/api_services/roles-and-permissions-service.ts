import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../environments/environment';
import {LoginService} from '../login.service';

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
  description: string;
  active: boolean;
  last_modified_by: string;
  last_modified_on: Date;
}
export interface NewOrPatchRole{
  name: string;
  description: string;
  active?: boolean;
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

  action: string;
  active: boolean;
  role_id: number;
  role: string;
  object_id: number;
  object: string;
  last_modified_by: string;
  last_modified_on: Date;
}
export interface ModifiedRolePermission{
  id: number;
  actions: string[];
  active: boolean;
  role_id: number;
  role_name: string;
  object_id: number;
  object_name: string;
  last_modified_by: string;
  last_modified_on: Date;
}

@Injectable({
  providedIn: 'root',
})
export class RolesAndPermissionsService {
  private apiUrl = environment.apiUrl;
  constructor(private http: HttpClient, private LoginService:LoginService) {
  }

  getRoles():Observable<RolePageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<RolePageInfo>(`${this.apiUrl}/api/roles`, {headers});
  }
  postRole(newRole:NewOrPatchRole){
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.post<Role>(`${this.apiUrl}/api/roles/`, newRole, {headers});
  }
  patchRole(roleId:number, newRole:NewOrPatchRole){
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.patch<Role>(`${this.apiUrl}/api/roles/${roleId}`, newRole, {headers});
  }
  getRolePermissions(): Observable<RolePermissionPageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<RolePermissionPageInfo>(`${this.apiUrl}/api/role_permissions`, {headers});

  }
}
