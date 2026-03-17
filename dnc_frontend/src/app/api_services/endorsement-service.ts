// src/app/api_services/endorsement-service.ts
import {Injectable, inject} from '@angular/core';
import {HttpClient, HttpHeaders, HttpParams} from '@angular/common/http';
import {Observable, map} from 'rxjs';
import {environment} from '../../environments/environment';
import {LoginService} from '../login.service';
import {AddableAutocompleteItem} from '../components/addable-autocomplete-component/addable-autocomplete-component';

// If you prefer, rename this to DateString
export type IsoDate = string; // "YYYY-MM-DD"
export type DecimalString = string; // e.g. "1234.50" (rust_decimal serde commonly uses strings)

export interface PageResponse<T> {
    page: number;      // 1-based
    page_size: number;
    total: number;
    items: T[];
}

/** Mirrors EndorsementResponse DTO */
export interface EndorsementResponse {
    id: number;
    hmo_id: number;
    endorsement_company_id: number;
    endorsement_type_id: number;
    agreement_corp_number: string | null;
    date_start: IsoDate;
    date_end: IsoDate;
    endorsement_billing_period_type_id: number;
    retainer_fee: DecimalString | null;
    remarks: string | null;
    endorsement_method: string | null;
}

/** Mirrors EndorsementListRow (joined names included) */
export interface EndorsementListRow {
    id: number;
    hmo_id: number;
    endorsement_company_id: number;
    endorsement_type_id: number;
    agreement_corp_number: string | null;
    date_start: IsoDate;
    date_end: IsoDate;
    endorsement_billing_period_type_id: number;
    retainer_fee: DecimalString | null;
    remarks: string | null;
    endorsement_method: string | null;

    hmo_name: string | null;
    company_name: string | null;
    type_name: string | null;
    billing_period_type_name: string | null;
}

/** Mirrors CreateEndorsementRequest DTO */
export interface CreateEndorsementRequest {
    hmo_id: number;
    endorsement_company_id: number;
    endorsement_type_id: number;
    agreement_corp_number?: string | null;
    date_start: IsoDate;
    date_end: IsoDate;
    endorsement_billing_period_type_id: number;
    retainer_fee?: DecimalString | null;
    remarks?: string | null;
    endorsement_method?: string | null;
}

/**
 * Mirrors PatchEndorsementRequest DTO semantics:
 * - plain Option<T> becomes "property?: T"
 * - Option<Option<String>> becomes "property?: string | null"
 *
 * IMPORTANT: In your Rust Patch DTO, agreement_corp_number is Option<Option<String>>
 * That means:
 *   - omit field => no change
 *   - send null => set to null
 *   - send string => set to string
 */
export interface PatchEndorsementRequest {
    hmo_id?: number;
    endorsement_company_id?: number;
    endorsement_type_id?: number;

    agreement_corp_number?: string | null;
    date_start?: IsoDate;
    date_end?: IsoDate;

    endorsement_billing_period_type_id?: number;

    // Your Rust DTO: Option<Option<Decimal>>; this matches the same “omit vs null vs value” behavior
    retainer_fee?: DecimalString | null;

    remarks?: string | null;
    endorsement_method?: string | null;
}

export interface ListQuery {
    page?: number;      // 1-based
    page_size?: number; // server clamps
}

export interface EndorsementCompanyOptions {
    id: number;
    name: string;
}

export interface CreateEndorsementCompanyRequest {
    name: string;
}

export interface CreateEndorsementCompanyResponse {
    company_id: number;
    company_name: string;
}

export interface EndorsementTypeOptions {
    endorsement_type_id: number;
    endorsement_type_name: string;
}

interface BillingFrequencyDb {
    id: number;
    name: string;
}

export interface BillingFrequencyOptions {
    billing_frequency_option_id: number;
    billing_frequency_option_name: string;
}

/** Mirrors EndorsementRateResponse DTO */
export interface EndorsementRateResponse {
    id: number;
    endorsement_id: number;
    dental_service_id: number;
    dental_service_name: string;
    dental_service_type_id: number;
    sort_index: number | null;
    record_tooth: boolean;
    active: boolean;
    rate: DecimalString;
}

/** Mirrors CreateEndorsementRateRequest DTO */
export interface CreateEndorsementRateRequest {
    dental_service_id: number;
    rate: DecimalString;
}

// ✅✅✅ ADDED: Mirrors UpdateEndorsementRatePutRequest DTO
export interface UpdateEndorsementRatePutRequest {
    dental_service_id: number;
    rate: DecimalString;
}

// ✅✅✅ ADDED: Mirrors UpdateEndorsementRatePatchRequest DTO
export interface UpdateEndorsementRatePatchRequest {
    dental_service_id?: number;
    rate?: DecimalString;
}

export interface EndorsementCountResponse {
    id: number;
    endorsement_id: number;
    dental_service_id: number;
    dental_service_name: string;
    dental_service_type_id: number;
    sort_index: number | null;
    record_tooth: boolean;
    active: boolean;
    count: number;
}

// ✅✅✅ ADDED: Mirrors CreateEndorsementCountRequest DTO
export interface CreateEndorsementCountRequest {
    dental_service_id: number;
    count: number;
}

// 🔵🔵🔵 ADDED: Mirrors UpdateEndorsementCountPutRequest DTO
export interface UpdateEndorsementCountPutRequest {
    dental_service_id: number;
    count: number;
}

// 🔵🔵🔵 ADDED: Mirrors UpdateEndorsementCountPatchRequest DTO
export interface UpdateEndorsementCountPatchRequest {
    dental_service_id?: number;
    count?: number;
}




@Injectable({providedIn: 'root'})
export class EndorsementService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    // Make sure environment.apiBaseUrl is something like "https://example.com/api"
    // and your routes are mounted at /endorsements under that base.
    private readonly baseUrl = `${environment.apiBaseUrl}`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({Authorization: `Bearer ${token}`});
    }

    /**
     *  GET /api/endorsement_companies
     */
    getEndorsementCompanies(): Observable<EndorsementCompanyOptions[]> {
        return this.http.get<EndorsementCompanyOptions[]>(`${this.baseUrl}/api/endorsements/companies`, {headers: this.authHeaders()});
    }

    /**
     * POST /api/endorsement_companies
     */
    createEndorsementCompany(name: string): Observable<AddableAutocompleteItem> {
        const payload: CreateEndorsementCompanyRequest = {
            name: name,
        }
        return this.http.post<CreateEndorsementCompanyResponse>(`${this.baseUrl}/api/endorsements/companies`, payload, {headers: this.authHeaders()})
            .pipe(map(response => ({
                id: String(response.company_id),
                label: response.company_name,
            })));
    }

    /**
     * GET /api/endorsement_types
     */
    getEndorsementTypes(): Observable<EndorsementTypeOptions[]> {
        return this.http.get<any[]>(`${this.baseUrl}/api/endorsement_types`, {headers: this.authHeaders()})
            .pipe(map(options => options.map(option => ({
                endorsement_type_id: option.id,
                endorsement_type_name: option.name,
            }))));
    }

    /**
     * GET /api/endorsement_billing_period_types
     */
    getEndorsementBillingPeriodTypes(): Observable<BillingFrequencyOptions[]> {
        return this.http.get<BillingFrequencyDb[]>(`${this.baseUrl}/api/endorsement_billing_period_types`,
            {headers: this.authHeaders()})
            .pipe(map(options => options.map(option => ({
                billing_frequency_option_id: option.id,
                billing_frequency_option_name: option.name,
            }))))
    }


    /**
     * GET /endorsements?page=1&page_size=25
     */
    getAll(query: ListQuery = {}): Observable<PageResponse<EndorsementListRow>> {
        const params = this.toParams(query);
        return this.http.get<PageResponse<EndorsementListRow>>(`${this.baseUrl}/api/endorsements`, {
            params,
            headers: this.authHeaders()
        });
    }

    /**
     * GET /endorsements/:id
     */
    get_endorsement_by_id(id: number): Observable<EndorsementResponse> {
        return this.http.get<EndorsementResponse>(`${this.baseUrl}/api/endorsements/${id}`, {headers: this.authHeaders()});
    }

    /**
     * POST /endorsements
     */
    create_endorsement(body: CreateEndorsementRequest): Observable<EndorsementResponse> {
        return this.http.post<EndorsementResponse>(`${this.baseUrl}/api/endorsements`, body, {headers: this.authHeaders()});
    }

    /**
     * PATCH /endorsements/:id
     */
    patch_endorsement(id: number, body: PatchEndorsementRequest): Observable<EndorsementResponse> {
        return this.http.patch<EndorsementResponse>(`${this.baseUrl}/api/endorsements/${id}`, body, {headers: this.authHeaders()});
    }

    /**
     * GET /api/endorsements/:endorsement_id/rates
     */
    getEndorsementRates(endorsementId: number): Observable<EndorsementRateResponse[]> {
        return this.http.get<EndorsementRateResponse[]>(
            `${this.baseUrl}/api/endorsements/${endorsementId}/rates`, {headers: this.authHeaders()});
    }

    /**
     * POST /api/endorsements/:endorsement_id/rates
     */
    createEndorsementRate(endorsementId: number, body: CreateEndorsementRateRequest): Observable<EndorsementRateResponse> {
        return this.http.post<EndorsementRateResponse>(`${this.baseUrl}/api/endorsements/${endorsementId}/rates`,
            body,
            {headers: this.authHeaders()}
        );
    }

    updateEndorsementRatePut(
        endorsementId: number,
        rateId: number,
        body: UpdateEndorsementRatePutRequest
    ): Observable<EndorsementRateResponse> {
        return this.http.put<EndorsementRateResponse>(
            `${this.baseUrl}/api/endorsements/${endorsementId}/rates/${rateId}`,
            body,
            {headers: this.authHeaders()}
        );
    }

    updateEndorsementRatePatch(
        endorsementId: number,
        rateId: number,
        body: UpdateEndorsementRatePatchRequest
    ): Observable<EndorsementRateResponse> {
        return this.http.patch<EndorsementRateResponse>(
            `${this.baseUrl}/api/endorsements/${endorsementId}/rates/${rateId}`,
            body,
            {headers: this.authHeaders()}
        );
    }

    // ✅✅✅ ADDED: GET /api/endorsements/:endorsement_id/counts
    getEndorsementCounts(endorsementId: number): Observable<EndorsementCountResponse[]> {
        return this.http.get<EndorsementCountResponse[]>(
            `${this.baseUrl}/api/endorsements/${endorsementId}/counts`,
            {headers: this.authHeaders()}
        );
    }

    // ✅✅✅ ADDED: POST /api/endorsements/:endorsement_id/counts
    createEndorsementCount(
        endorsementId: number,
        body: CreateEndorsementCountRequest
    ): Observable<EndorsementCountResponse> {
        return this.http.post<EndorsementCountResponse>(
            `${this.baseUrl}/api/endorsements/${endorsementId}/counts`,
            body,
            {headers: this.authHeaders()}
        );
    }
    // 🔵🔵🔵 ADDED: PUT /api/endorsements/:endorsement_id/counts/:count_id
    updateEndorsementCountPut(
        endorsementId: number,
        countId: number,
        body: UpdateEndorsementCountPutRequest
    ): Observable<EndorsementCountResponse> {
        return this.http.put<EndorsementCountResponse>(
            `${this.baseUrl}/api/endorsements/${endorsementId}/counts/${countId}`,
            body,
            {headers: this.authHeaders()}
        );
    }

    // 🔵🔵🔵 ADDED: PATCH /api/endorsements/:endorsement_id/counts/:count_id
    updateEndorsementCountPatch(
        endorsementId: number,
        countId: number,
        body: UpdateEndorsementCountPatchRequest
    ): Observable<EndorsementCountResponse> {
        return this.http.patch<EndorsementCountResponse>(
            `${this.baseUrl}/api/endorsements/${endorsementId}/counts/${countId}`,
            body,
            {headers: this.authHeaders()}
        );
    }



    // --- helpers

    private toParams(q: ListQuery): HttpParams {
        let params = new HttpParams();
        if (q.page != null) params = params.set('page', String(q.page));
        if (q.page_size != null) params = params.set('page_size', String(q.page_size));
        return params;
    }
}
