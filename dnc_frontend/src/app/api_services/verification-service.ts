import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {map, Observable} from 'rxjs';

export interface VerificationLookupResponse {
    verification_id: number;
    date_created: string; // ISO datetime string from Rust DateTimeWithTimeZone
    dentist_id: number;
    dentist_name: string;
    master_list_member_id: number;
    master_list_member_name: string;
    dental_service_id: number;
    dental_service_name: string;
    record_tooth:boolean,
    status_id: number;
    status_name: string;
    approval_code: string | null;
    approved_by: string | null;
    approval_date: string | null;
}
export interface ExtendedVerificationLookupResponse extends VerificationLookupResponse {
    approval_string: string | null;
}

export interface CreateVerificationRequest {
    dentist_id: number;
    member_id: number;
    dental_service_id: number;
}

export interface CreateVerificationResponse {
    id: number;
    date_created: string;               // ISO datetime from backend
    created_by: string;
    dentist_id: number;
    member_id: number;
    dental_service_id: number;
    date_service_performed: string | null; // ISO date or null
    status_id: number;
    approved_by: string | null;
    approval_date: string | null;       // ISO datetime or null
    approval_code: string | null;
}

export interface GetApprovalCodeResponse {
    approval_code: string | null;
}

@Injectable({
  providedIn: 'root',
})
export class VerificationService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    // Make sure environment.apiBaseUrl is something like "https://example.com/api"
    // and your routes are mounted at /endorsements under that base.
    private readonly baseUrl = `${environment.apiBaseUrl}/api/verifications`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }
    getVerifications(): Observable<ExtendedVerificationLookupResponse[]> {
        return this.http.get<VerificationLookupResponse[]>(`${this.baseUrl}`, {headers: this.authHeaders()})
            .pipe(
                map((rows)=>
                    rows.map((row):ExtendedVerificationLookupResponse =>({
                            ...row,
                            approval_string:
                               row.status_id===99 &&
                                row.approved_by &&
                                row.approval_date &&
                                row.approval_code ?
                                    `By:${row.approved_by} (on ${new Date(row.approval_date).toLocaleDateString()}) with code: ${row.approval_code}`
                                   : null,
                        })))
            );
    }

    createVerification(
        payload: CreateVerificationRequest
    ): Observable<CreateVerificationResponse> {
        return this.http.post<CreateVerificationResponse>(
            `${this.baseUrl}`,
            payload,
            { headers: this.authHeaders() }
        );
    }

    cancelVerification(id: number): Observable<any> {
        return this.http.post<any>(`${this.baseUrl}/${id}/cancel`,{}, {headers: this.authHeaders()});
    }


    requestApprovalCode(validation_id:number, date_service_performed: string): Observable<GetApprovalCodeResponse> {
        const payload = {date_service_performed};
        return this.http.post<GetApprovalCodeResponse>(`${this.baseUrl}/${validation_id}/approval_code`,payload, {headers: this.authHeaders()});
    }
}
