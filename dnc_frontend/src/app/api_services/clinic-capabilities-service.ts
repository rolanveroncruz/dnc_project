import { Injectable } from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../environments/environment';
import {LoginService} from '../login.service';


export interface ClinicCapabilitiesPageInfo{
  page: number;
  perSize: number;
  total_items: number;
  total_pages: number;
  items: ClinicCapability[];
}
export interface ClinicCapability{
  id: number;
  name: string;
  last_modified_by: string;
  last_modified_on: Date;
}
@Injectable({
  providedIn: 'root',
})
export class ClinicCapabilitiesService {
  private apiUrl = environment.apiUrl;
  constructor(private http: HttpClient, private LoginService:LoginService) { }

  getClinicCapabilities():Observable<ClinicCapabilitiesPageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<ClinicCapabilitiesPageInfo>(`${this.apiUrl}/api/clinic_capabilities?`, {headers});
  }

}
