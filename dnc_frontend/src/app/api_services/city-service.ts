// src/app/api_services/city-service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface PageResponse<T> {
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
  items: T[];
}

export interface CityRow {
  id: number;
  name: string;
  state_id: number;
}

export interface CreateCityRequest {
  name: string;
  state_id: number;
}

export interface PatchCityRequest {
  name?: string | null;
  state_id?: number | null;
}

@Injectable({ providedIn: 'root' })
export class CityService {
  private readonly http = inject(HttpClient);

  // backend routes:
  // GET    /api/cities?page=&page_size=&state_id=&region_id=
  // GET    /api/cities/:id
  // POST   /api/cities
  // PATCH  /api/cities/:id
  private readonly baseUrl = `${environment.apiUrl}/cities`;

  getCities(opts?: {
    page?: number;
    pageSize?: number;
    stateId?: number;
    regionId?: number;
  }): Observable<PageResponse<CityRow>> {
    let params = new HttpParams();

    if (opts?.page != null) params = params.set('page', String(opts.page));
    if (opts?.pageSize != null) params = params.set('page_size', String(opts.pageSize));

    // filters
    if (opts?.stateId != null) params = params.set('state_id', String(opts.stateId));
    if (opts?.regionId != null) params = params.set('region_id', String(opts.regionId));

    return this.http.get<PageResponse<CityRow>>(this.baseUrl, { params });
  }

  getCityById(id: number): Observable<CityRow> {
    return this.http.get<CityRow>(`${this.baseUrl}/${id}`);
  }

  createCity(payload: CreateCityRequest): Observable<CityRow> {
    return this.http.post<CityRow>(this.baseUrl, payload);
  }

  patchCity(id: number, payload: PatchCityRequest): Observable<CityRow> {
    return this.http.patch<CityRow>(`${this.baseUrl}/${id}`, payload);
  }
}
