import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';

export interface UtilizationReportRow {
    source: string,
    id: number,
    date_created: string,
    dentist_id: number,
    dentist_name: string,
    company_id: number,
    company_name: string,
    member_id: number,
    member_account_number: string,
    member_name: string,
    dental_service_name: string,
    date_service_performed: string | null,
    tooth: string | null,
}

@Injectable({
  providedIn: 'root',
})
export class UtilizationReportsService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    // Make sure environment.apiBaseUrl is something like "https://example.com/api"
    // and your routes are mounted at /endorsements under that base.
    private readonly baseUtilizationReportsUrl = `${environment.apiBaseUrl}/api/utilization_reports`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }

    getUtilizationReportForCompany(companyId: number): Observable<UtilizationReportRow[]> {
        return this.http.get<UtilizationReportRow[]>(`${this.baseUtilizationReportsUrl}/company/${companyId}`, {headers: this.authHeaders()});
    }
    downloadUtilizationReportForCompany(companyId:number){
        return this.http.get(`${this.baseUtilizationReportsUrl}/company/${companyId}/download`, {headers: this.authHeaders(), responseType: 'blob'});
    }

}
