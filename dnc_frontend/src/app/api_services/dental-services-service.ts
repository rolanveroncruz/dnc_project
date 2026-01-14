import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../environments/environment';
import {LoginService} from '../login.service';

export interface DentalServicesPageInfo{
  page: number;
  perSize: number;
  total_items: number;
  total_pages: number;
  items: DentalService[];
}
export interface DentalService {
  id: number;
  name: string;
  type_id: number;
  type_name: string;
  record_tooth: boolean;
  active: boolean;
  last_modified_by: string;
  last_modified_on: Date;
}
@Injectable({
  providedIn: 'root',
})
export class DentalServicesService {
  private apiUrl = environment.apiUrl;
  constructor(private http: HttpClient, private LoginService:LoginService) { }

  getDentalServices():Observable<DentalServicesPageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<DentalServicesPageInfo>(`${this.apiUrl}/api/dental_services?`, {headers});
  }
  postDentalService(dentalService:DentalService){
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.post<DentalService>(`${this.apiUrl}/api/dental_services/`, dentalService, {headers});
  }
  patchDentalService(dentalServiceId:number, dentalService:DentalService){
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.patch<DentalService>(`${this.apiUrl}/api/dental_services/${dentalServiceId}`, dentalService, {headers});
  }
}

