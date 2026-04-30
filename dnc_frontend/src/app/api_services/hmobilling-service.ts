import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';

export interface HMOBillingRow {
    statement_of_account_no: string,
    company_name: string,
    agreement_corp_number: string | null,
    total_master_list_members: number,
    billing_period_type_name: string,
    dental_benefits: string,
    effectivity_period: string,
    retainer_fee: string | null,
}

@Injectable({
  providedIn: 'root',
})
export class HMOBillingService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    // Make sure environment.apiBaseUrl is something like "https://example.com/api"
    // and your routes are mounted at /endorsements under that base.
    private readonly baseHMOBillingUrl = `${environment.apiBaseUrl}/api/hmo_billing`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }

    getHMOBillingForHMO(hmo_id:number): Observable<HMOBillingRow[]>{
        return this.http.get<HMOBillingRow[]>(`${this.baseHMOBillingUrl}/${hmo_id}`, {headers: this.authHeaders()});
    }
    downloadBillingReportForHMO(hmoId:number){
        return this.http.get(`${this.baseHMOBillingUrl}/${hmoId}/download`, {headers: this.authHeaders(), responseType: 'blob'});
    }

}
