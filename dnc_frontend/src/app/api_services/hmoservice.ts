import { Injectable } from '@angular/core';
import {environment} from '../../environments/environment';
import {HttpClient} from '@angular/common/http';
import {EMPTY, Observable} from 'rxjs';
import {LoginService} from '../login.service';

export interface HMOPageInfo{
  page: number;
  perSize: number;
  total_items: number;
  total_pages: number;
  items: HMO[];
}
export type HMOEditable = Pick<
  HMO,
  'short_name' | 'long_name' | 'address' | 'tax_account_number' | 'contact_nos' | 'active'
>;
export interface HMO{
  id: number;
  short_name: string;
  long_name: string;
  address: string;
  tax_account_number: string;
  contact_nos: string;
  active: boolean;
  last_endorsement_date: Date | null;
  last_collection_date: Date | null;
  last_modified_by: string;
  last_modified_on: Date;
}

@Injectable({
  providedIn: 'root',
})
export class HMOService {
  private apiUrl = environment.apiUrl;
  constructor(private http: HttpClient, private LoginService:LoginService) {}

  getHMOs():Observable<HMOPageInfo>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<HMOPageInfo>(`${this.apiUrl}/api/hmos`, {headers});
  }

  getHMOById(id:number):Observable<HMO>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<HMO>(`${this.apiUrl}/api/hmos/${id}`, {headers});
  }
  postHMO(hmo:HMOEditable){
    console.log("In HMO Service, Post HMO:", hmo);
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.post<HMO>(`${this.apiUrl}/api/hmos/`, hmo, {headers});
  }
  patchHMO(hmoId:number|null, hmo:HMOEditable){
    console.log("In HMO Service, Patch HMO:", hmo);
    if (hmoId === null) return EMPTY;
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.patch<HMO>(`${this.apiUrl}/api/hmos/${hmoId}`, hmo, {headers});
  }

}
