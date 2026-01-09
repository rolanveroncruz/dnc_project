import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../environments/environment';
import {LoginService} from '../login.service';
import {DentalServicesPageInfo} from './dental-services-service';

export interface DentalServiceTypesPageInfo{
  page: number;
  perSize: number;
  total_items: number;
  total_pages: number;
  items: DentalServiceType[];
}
export interface DentalServiceType {
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
export class DentalServiceTypesService {
  private apiUrl = environment.apiUrl;
  constructor(private http: HttpClient, private LoginService:LoginService) { }

  getDentalServiceTypes():Observable<DentalServiceTypesPageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<DentalServiceTypesPageInfo>(`${this.apiUrl}/api/dental_service_types`, {headers});
  }

}
