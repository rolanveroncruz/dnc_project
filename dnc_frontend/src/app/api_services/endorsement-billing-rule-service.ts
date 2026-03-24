// src/app/api_services/endorsement-billing-rules.service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import { LoginService } from '../login.service';

export interface EndorsementBillingRuleResponse {
    id: number;
    endorsement_id: number;
    min_count: number;
    max_count: number;
    rate: number | string;
}

export interface CreateEndorsementBillingRuleRequest {
    endorsement_id: number;
    min_count: number;
    max_count: number;
    rate: number | string;
}

export interface PatchEndorsementBillingRuleRequest {
    min_count?: number;
    max_count?: number;
    rate?: number | string;
}

@Injectable({
    providedIn: 'root',
})
export class EndorsementBillingRulesService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private readonly baseUrl = `${environment.apiUrl}/api`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({
            Authorization: `Bearer ${token}`,
        });
    }

    getBillingRulesForEndorsementId(
        endorsementId: number
    ): Observable<EndorsementBillingRuleResponse[]> {
        return this.http.get<EndorsementBillingRuleResponse[]>(
            `${this.baseUrl}/endorsement/${endorsementId}/billing-rules`,
            { headers: this.authHeaders() }
        );
    }

    createBillingRule(
        payload: CreateEndorsementBillingRuleRequest
    ): Observable<EndorsementBillingRuleResponse> {
        return this.http.post<EndorsementBillingRuleResponse>(
            `${this.baseUrl}/endorsement/${payload.endorsement_id}/billing-rules`,
            payload,
            { headers: this.authHeaders() }
        );
    }

    patchBillingRule(
        id: number,
        endorsementId: number,
        payload: PatchEndorsementBillingRuleRequest
    ): Observable<EndorsementBillingRuleResponse> {
        return this.http.patch<EndorsementBillingRuleResponse>(
            `${this.baseUrl}/endorsements/${endorsementId}/billing-rules/${id}`,
            payload,
            { headers: this.authHeaders() }
        );
    }

    deleteBillingRule(endorsementId: number, id: number): Observable<void> {
        return this.http.delete<void>(
            `${this.baseUrl}/endorsements/${endorsementId}/endorsement-billing-rules/${id}`,
            { headers: this.authHeaders() }
        );
    }
}
