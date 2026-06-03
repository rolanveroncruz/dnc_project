import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';

export interface GeneratedBillingReportResponse {
    id: number,
    report_type_id: number,
    file_name: string,
    date_generated: string | null,
}

@Injectable({
  providedIn: 'root',
})
export class HMOBillingService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    // Make sure environment.apiBaseUrl is something like "https://example.com/api"
    // and your routes are mounted at /endorsements under that base.
    private readonly baseUrl = `${environment.apiBaseUrl}`;
    private readonly baseHMOBillingUrl = `${this.baseUrl}/api/hmo_billing`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }

    getHMOBillingReports(): Observable<GeneratedBillingReportResponse[]>{
        return this.http.get<GeneratedBillingReportResponse[]>(`${this.baseHMOBillingUrl}/`, {headers: this.authHeaders()});
    }
    downloadGeneratedReport(filename:string){
        return this.http.get(`${this.baseHMOBillingUrl}/download/${filename}`, {headers: this.authHeaders(), responseType: 'blob'});
    }
    generateHMOBillingReports(): Observable<any>{
        return this.http.get(`${this.baseUrl}/generate_hmo_billings`, {headers: this.authHeaders()});
    }

}
