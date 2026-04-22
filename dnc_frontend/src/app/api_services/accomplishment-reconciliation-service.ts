import {inject, Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {LoginService} from '../login.service';
import {environment} from '../../environments/environment';
import {Observable} from 'rxjs';
import {VerificationLookupResponse} from './verification-service';

export interface CreateAccReconciliationRequest {
    dentist_id: number,
    member_id: number,
    dental_service_id: number,
    date_service_performed: Date | null,
    approval_code: string | null,
    tooth_id: string | null,
    tooth_service_type_id: number | null,
    tooth_surface_id: number | null,
}


export interface DoneVerificationResponse {
    id: number,
    date_created: string,
    dentist_name: string,
    member_name: string,
    dental_service_name: string,
    agreement_corp_number: string,
    company_name: string,
    date_service_performed: string,
    tooth_id: string | null,
    tooth_surface_name:string | null,
    tooth_service_type_name: string | null,
    approval_code: string,
    approval_date: string |null,
    is_reconciled: boolean,
    reconciled_by: string | null,
    reconciliation_date: string | null,
}

@Injectable({
  providedIn: 'root',
})
export class AccomplishmentReconciliationService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    // Make sure environment.apiBaseUrl is something like "https://example.com/api"
    // and your routes are mounted at /endorsements under that base.
    private readonly baseUrl = `${environment.apiBaseUrl}/api/acc_recon`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }

    getDoneVerifications(): Observable<DoneVerificationResponse[]> {
        return this.http.get<DoneVerificationResponse[]>(`${this.baseUrl}/verifications`, {headers: this.authHeaders()})
    }
    reconcileVerification(verification_id:number): Observable<DoneVerificationResponse> {
        return this.http.post<DoneVerificationResponse>(`${this.baseUrl}/${verification_id}/reconcile`,
            {},
            {headers: this.authHeaders()}
        )
    }

}
