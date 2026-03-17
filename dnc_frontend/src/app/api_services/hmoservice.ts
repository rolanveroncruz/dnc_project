import { Injectable } from '@angular/core';
import {environment} from '../../environments/environment';
import {HttpClient} from '@angular/common/http';
import {EMPTY, Observable,map} from 'rxjs';
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
  'short_name' | 'long_name' | 'address' | 'tax_account_number' | 'contact_nos' | 'expect_a_master_list' | 'active'
>;
export interface HMO{
  id: number;
  short_name: string;
  long_name: string;
  address: string;
  tax_account_number: string;
  contact_nos: string;
  expect_a_master_list: boolean;
  active: boolean;
  last_endorsement_date: Date | null;
  last_collection_date: Date | null;
  last_modified_by: string;
  last_modified_on: Date;
}
export interface HMOOptions{
    id:number,
  short_name: string;
  long_name: string;
  expect_a_master_list: boolean;
}
export interface EndorsementWithLookupsResponse {
    id: number;
    hmo_id: number;
    endorsement_company_id: number;
    endorsement_company_name: string;
    endorsement_type_id: number;
    endorsement_type_name: string;
    agreement_corp_number: string | null;
    date_start: string;
    date_end: string;
    endorsement_billing_period_type_id: number;
    endorsement_billing_period_type_name: string;
    retainer_fee: number | null;
    remarks: string | null;
    endorsement_method: string | null;
    is_active: boolean;
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
  getHMOOptions():Observable<HMOOptions[]>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<HMOPageInfo>(`${this.apiUrl}/api/hmos`, {headers})
        .pipe(
            map((page)=> page.items.map
            (
                (hmo)=>
                    ({id:hmo.id,
                    short_name:hmo.short_name,
                    long_name:hmo.long_name,
                    expect_a_master_list:hmo.expect_a_master_list,})
            )
            )
        );
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

  getEndorsements(hmoId:number):Observable<EndorsementWithLookupsResponse[]>{
    let token = this.LoginService.token();
    const headers = {'Authorization': `Bearer ${token}`};
    return this.http.get<EndorsementWithLookupsResponse[]>(`${this.apiUrl}/api/hmos/${hmoId}/endorsements`, {headers});
  }
}
