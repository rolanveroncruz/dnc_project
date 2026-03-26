import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';
import {EndorsementCompanyOptions} from './endorsement-service';

export interface VerificationLookupResponse {
    verification_id: number;
    date_created: string; // ISO datetime string from Rust DateTimeWithTimeZone
    dentist_id: number;
    dentist_name: string;
    master_list_member_id: number;
    master_list_member_name: string;
    dental_service_id: number;
    dental_service_name: string;
    status_id: number;
    status_name: string;
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
    getVerifications(): Observable<VerificationLookupResponse[]> {
        return this.http.get<VerificationLookupResponse[]>(`${this.baseUrl}`, {headers: this.authHeaders()});
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
}
