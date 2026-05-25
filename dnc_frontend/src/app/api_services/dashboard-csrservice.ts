import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';

export interface CsrVerificationActivityRow {
    user_id: number,
    name: string,
    email: string,
    created_count: number,
    approved_count: number,
    reconciled_count: number,
}
export interface CsrVerificationActivityUnitRow {
    period_start: string,
    user_id: number,
    name: string,
    email: string,
    created_count: number,
    approved_count: number,
    reconciled_count: number,
}



@Injectable({
  providedIn: 'root',
})
export class DashboardCSRService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private readonly baseUrl = `${environment.apiBaseUrl}/api`;
    private readonly baseDashboardUrl = `${environment.apiBaseUrl}/api/dashboard`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }


    getCSRVerificationTotals(start_date: string, end_date:string,): Observable<CsrVerificationActivityRow[]>{

        return this.http.get<CsrVerificationActivityRow[]>(`${this.baseDashboardUrl}/csr_verification_activity`,{
            headers: this.authHeaders(),
            params: {
                start_date: start_date,
                end_date: end_date,
            }
        });
    };

    getCSRVerificationUnitTotals(start_date: string, end_date:string,): Observable<CsrVerificationActivityUnitRow[]>{
        return this.http.get<CsrVerificationActivityUnitRow[]>(`${this.baseDashboardUrl}/csr_verification_activity_unit_counts`,{
            headers: this.authHeaders(),
            params: {
                start_date: start_date,
                end_date: end_date,
                unit: 'day',
            }
        });

    }
}
